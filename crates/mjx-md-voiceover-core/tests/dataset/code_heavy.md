# System Architecture & API Reference

This document describes the high-performance async runtime and database integration.

## Installation

Run the following command to install the package:

```bash
cargo add mjx-md-voiceover --features full
npm install @mjx/md-voiceover
pip install mjx-md-voiceover
```

## Rust Implementation

```rust
use std::sync::Arc;

pub struct VoiceEngine {
    buffer_size: usize,
}

impl VoiceEngine {
    pub fn new(capacity: usize) -> Self {
        Self { buffer_size: capacity }
    }

    pub fn process_stream(&self, input: &str) -> String {
        format!("Processed: {}", input)
    }
}
```

## Python Integration

```python
import asyncio
from mjx_md_voiceover import VoiceEngine

async def main():
    engine = VoiceEngine(capacity=1024)
    result = await engine.process_async("# Hello")
    print(f"Result: {result}")

if __name__ == "__main__":
    asyncio.run(main())
```

## SQL Schema Definition

```sql
CREATE TABLE voice_sessions (
    session_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    markdown_bytes INT NOT NULL,
    latency_ms NUMERIC(6, 3) NOT NULL
);
```

Use `engine.process_stream(&data)` inside your main thread loop.
