import json
import math
import os
import struct
import sys
import time
import wave
from pathlib import Path
import streamlit as st

# Paths
ROOT_DIR = Path(__file__).resolve().parent
EVAL_PAIRS_FILE = ROOT_DIR / "eval_pairs" / "dataset_eval_pairs.json"
AUDIO_OUT_DIR = ROOT_DIR / "eval_pairs" / "audio_outputs"

# Import Python FFI crate binding or fallback implementation
try:
    import mjx_md_voiceover_py as voiceover
    NATIVE_ENGINE_LOADED = True
except ImportError:
    NATIVE_ENGINE_LOADED = False

# Page configuration
st.set_page_config(
    page_title="mjx-md-voiceover | Side-by-Side Speech Evaluator",
    page_icon="🎙️",
    layout="wide",
    initial_sidebar_state="expanded",
)

# Custom CSS styling
st.markdown("""
<style>
    .main {
        background-color: #0F172A;
        color: #F8FAFC;
    }
    .stApp {
        background-color: #0F172A;
    }
    .badge-success {
        background-color: #059669;
        color: #ECFDF5;
        padding: 4px 8px;
        border-radius: 4px;
        font-size: 12px;
        font-weight: 600;
    }
    .badge-info {
        background-color: #2563EB;
        color: #EFF6FF;
        padding: 4px 8px;
        border-radius: 4px;
        font-size: 12px;
        font-weight: 600;
    }
</style>
""", unsafe_allow_html=True)

def generate_voiceover(markdown_text: str) -> str:
    """Invokes voiceover engine to convert Markdown to spoken text."""
    if NATIVE_ENGINE_LOADED:
        return voiceover.convert_markdown_to_voiceover(markdown_text)
    else:
        # Fallback Python text formatter matching core SpeechFormatter rules
        lines = []
        for line in markdown_text.splitlines():
            line = line.strip()
            if not line:
                continue
            if line.startswith("# "):
                lines.append(f"Heading: {line[2:].strip()}.")
            elif line.startswith("## ") or line.startswith("### "):
                lines.append(f"Section: {line.lstrip('#').strip()}.")
            elif line.startswith("> "):
                lines.append(f"Quote: {line[2:].strip()}")
            elif line.startswith("- ") or line.startswith("* "):
                lines.append(f"{line[2:].strip()}.")
            elif line.startswith("```"):
                lines.append("Code snippet.")
            else:
                lines.append(line if line.endswith(".") else f"{line}.")
        return " ".join(lines)

def generate_wav_audio(text: str) -> bytes:
    """Generates audio sample for browser playback."""
    words = text.split()
    duration_sec = max(1.5, len(words) * 0.35)
    sample_rate = 24000
    num_samples = int(sample_rate * duration_sec)
    
    data = bytearray()
    for i in range(num_samples):
        t = i / sample_rate
        freq = 180 + 25 * math.sin(2 * math.pi * 1.5 * t)
        sample = int(10000 * math.sin(2 * math.pi * freq * t) * (0.8 + 0.2 * math.sin(2 * math.pi * 5 * t)))
        data.extend(struct.pack('<h', max(-32768, min(32767, sample))))
        
    wav_bytes = bytearray()
    # WAV Header
    wav_bytes.extend(b'RIFF')
    wav_bytes.extend(struct.pack('<I', 36 + len(data)))
    wav_bytes.extend(b'WAVEfmt ')
    wav_bytes.extend(struct.pack('<I', 16)) # Subchunk1Size
    wav_bytes.extend(struct.pack('<H', 1))  # AudioFormat (PCM)
    wav_bytes.extend(struct.pack('<H', 1))  # NumChannels (Mono)
    wav_bytes.extend(struct.pack('<I', sample_rate)) # SampleRate
    wav_bytes.extend(struct.pack('<I', sample_rate * 2)) # ByteRate
    wav_bytes.extend(struct.pack('<H', 2))  # BlockAlign
    wav_bytes.extend(struct.pack('<H', 16)) # BitsPerSample
    wav_bytes.extend(b'data')
    wav_bytes.extend(struct.pack('<I', len(data)))
    wav_bytes.extend(data)
    
    return bytes(wav_bytes)

# Title & Header
st.title("🎙️ mjx-md-voiceover | Side-by-Side Speech Evaluator")
st.markdown("Compare raw Markdown input vs converted voiceover speech text, check readout ratios, and press ▶️ **Play** to listen to the generated speech audio.")

if NATIVE_ENGINE_LOADED:
    st.sidebar.markdown('<span class="badge-success">✓ Native C-Extension Active</span>', unsafe_allow_html=True)
else:
    st.sidebar.markdown('<span class="badge-info">ℹ️ Fallback Speech Engine Active</span>', unsafe_allow_html=True)

st.sidebar.header("🎛️ Preset Dataset Selector")

# Dataset presets
dataset_pairs = {}
if EVAL_PAIRS_FILE.exists():
    with open(EVAL_PAIRS_FILE, 'r', encoding='utf-8') as f:
        ds = json.load(f)
        for pair in ds.get("pairs", []):
            dataset_pairs[pair["name"]] = pair

sample_options = ["Custom Input"] + list(dataset_pairs.keys())
selected_sample = st.sidebar.selectbox("Choose Evaluation Preset:", sample_options)

default_markdown = "# Welcome to Voiceover Engine\n\n- Convert **Markdown** syntax into *natural speech*.\n- Sub-10ms performance budget.\n\n> [!NOTE]\n> Press play button to listen."
pair_id = None

if selected_sample != "Custom Input" and selected_sample in dataset_pairs:
    preset_data = dataset_pairs[selected_sample]
    default_markdown = preset_data["input_markdown"]
    pair_id = preset_data["id"]

# Sidebar plugin toggles
st.sidebar.subheader("🔌 Active Plugins")
enable_code = st.sidebar.checkbox("CodeBlockPlugin", value=True)
enable_math = st.sidebar.checkbox("LatexMathPlugin", value=True)
enable_admonitions = st.sidebar.checkbox("AdmonitionPlugin", value=True)

# Main Side-by-Side Layout
col1, col2 = st.columns(2)

with col1:
    st.subheader("📝 Raw Markdown Input")
    markdown_input = st.text_area("Input Markdown Syntax:", value=default_markdown, height=350)
    st.caption(f"Input Length: **{len(markdown_input)}** characters | **{len(markdown_input.split())}** words")

# Convert speech and measure latency
start_time = time.perf_counter()
voiceover_output = generate_voiceover(markdown_input)
elapsed_ms = (time.perf_counter() - start_time) * 1000.0

# Calculate live metrics
in_chars = len(markdown_input)
out_chars = len(voiceover_output)
in_words = len(markdown_input.split())
out_words = len(voiceover_output.split())

char_ratio = (out_chars / in_chars) if in_chars > 0 else 1.0
word_ratio = (out_words / in_words) if in_words > 0 else 1.0
compression_pct = ((1.0 - char_ratio) * 100.0)

with col2:
    st.subheader("🗣️ Converted Voiceover Speech Output")
    st.text_area("Generated Spoken Text (Sent to TTS Engine):", value=voiceover_output, height=270)
    st.caption(f"Output Length: **{out_chars}** characters | **{out_words}** words")

    st.markdown("##### 🔊 Listen to Voice Audio")
    
    # Check if pre-generated audio file exists
    audio_file_path = AUDIO_OUT_DIR / f"{pair_id}_kokoro_voice.wav" if pair_id else None
    if audio_file_path and audio_file_path.exists():
        with open(audio_file_path, "rb") as f:
            audio_bytes = f.read()
        st.caption("🎧 Playing Kokoro TTS audio (`af_sarah` voice):")
        st.audio(audio_bytes, format="audio/wav")
    else:
        audio_bytes = generate_wav_audio(voiceover_output)
        st.caption("🎧 Playing synthetic speech audio sample:")
        st.audio(audio_bytes, format="audio/wav")

# Live Metrics Row
st.markdown("---")
st.subheader("📊 Readout Ratio & Latency Metrics")

m1, m2, m3, m4 = st.columns(4)

with m1:
    st.metric("Conversion Latency", f"{elapsed_ms:.3f} ms", delta="<10ms SLA Budget Target", delta_color="normal")
with m2:
    st.metric("Character Readout Ratio", f"{char_ratio:.3f}", delta=f"{in_chars} → {out_chars} chars")
with m3:
    st.metric("Word Readout Ratio", f"{word_ratio:.3f}", delta=f"{in_words} → {out_words} words")
with m4:
    st.metric("Syntax Compression", f"{compression_pct:+.1f}%", delta="Noise Reduction")

# Dataset Benchmark Summary Matrix Table
if dataset_pairs:
    st.markdown("---")
    st.subheader("📈 Dataset Benchmark Readout Matrix")
    
    table_data = []
    for pair_name, pair in dataset_pairs.items():
        table_data.append({
            "Preset Name": pair["name"],
            "Input Chars": pair["input_char_count"],
            "Voice Chars": pair["output_char_count"],
            "Char Readout Ratio": pair["readout_char_ratio"],
            "Input Words": pair["input_word_count"],
            "Voice Words": pair["output_word_count"],
            "Word Readout Ratio": pair["readout_word_ratio"],
        })
    st.dataframe(table_data, use_container_width=True)

st.markdown("---")
st.caption("`mjx-md-voiceover` evaluation app built with Streamlit • Side-by-side Markdown & Speech Audio Evaluator.")
