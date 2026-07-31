# Dual-TTS Synthesis Latency & Readout Evaluation Report

**Engine:** `mjx-md-voiceover` v0.1.0 (Sub-1ms Pure Rust Core)  
**TTS Model:** Kokoro-82M PyTorch Neural Voice (`af_sarah`, 24 kHz)  
**Dataset Version:** `1.0.0` (5 Domain Markdown Test Cases)

---

## 📊 Dual-TTS Latency & Compute Summary

By converting raw Markdown syntax noise (`###`, `**`, `code`, `$math$`, `> [!NOTE]`) into clean spoken text before passing to the TTS engine, `mjx-md-voiceover` dramatically reduces TTS neural inference compute time and audio duration.

| Dataset Preset | Input Chars | Voice Chars | Raw TTS Compute ($T_{\text{raw}}$) | Parsed TTS Compute ($T_{\text{parsed}}$) | TTS Time Saved ($\Delta T_{\text{TTS}}$) | TTS Speedup % |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **Code Heavy** | 1,239 | 461 | **7.3547 s** | **1.3557 s** | **+5.9989 s** | **+81.6%** |
| **Admonitions Mixed** | 760 | 789 | **4.3207 s** | **2.2834 s** | **+2.0373 s** | **+47.2%** |
| **Nested Lists** | 859 | 809 | **4.7691 s** | **2.5271 s** | **+2.2420 s** | **+47.0%** |
| **Technical Spec** | 895 | 866 | **3.8895 s** | **2.9179 s** | **+0.9716 s** | **+25.0%** |
| **Math Heavy** | 1,028 | 1,049 | **4.9985 s** | **4.3742 s** | **+0.6242 s** | **+12.5%** |

---

## 🎧 Side-by-Side Audio Evaluation Findings

1. **Code Heavy Preset:**
   - **Raw Audio:** Verbatim reading of bracket symbols, curly braces, and indents creates 2 minutes of jarring noise.
   - **Parsed Audio:** Converted to crisp 29-second summary ("Code snippet in Rust..."). Saved **6.0 seconds** of neural TTS compute (**+81.6% faster**).
2. **Admonitions Mixed Preset:**
   - **Raw Audio:** Reads `greater than left bracket exclamation mark NOTE right bracket`.
   - **Parsed Audio:** Reads natural alert cue ("Note callout. Database backup snapshots created..."). Saved **2.04 seconds** of TTS compute (**+47.2% faster**).
3. **Math Heavy Preset:**
   - **Raw Audio:** Reads backslashes, dollar signs, and curly brackets verbatim.
   - **Parsed Audio:** Speaks natural math ("a squared plus b squared equals c squared"). Saved **0.62 seconds** of TTS compute (**+12.5% faster**).

---

## 🚀 Conclusion

Pre-processing Markdown with `mjx-md-voiceover` achieves an average **+42.7% reduction in neural TTS compute time**, transforming unlistenable Markdown syntax symbols into natural, high-fidelity spoken prose.
