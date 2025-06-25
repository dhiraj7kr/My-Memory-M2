#!/bin/bash
cd summarizer/llama.cpp
echo "$1" | ./main -m models/llama-7B.ggmlv3.q4_0.bin
