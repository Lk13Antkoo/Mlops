# This lap is used to invoke local model

## I. Describe
This lab is for mimicing the openai API key, how to interact with local LM.

## II. Step
1. Install dependencies
```bash
pip install - r requirements.txt
```
2. Run local model
```bash 
    ./build/bin/llama-server -m ./gemma-1.1-7b-it.Q4_K_M.gguf -ngl 20
```
3. Run application
```bash
    python ./main.py 
```