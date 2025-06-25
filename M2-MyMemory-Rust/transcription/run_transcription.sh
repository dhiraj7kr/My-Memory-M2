#!/bin/bash
cd transcription/whisper.cpp
./main -f $1 -m models/ggml-base.en.bin
