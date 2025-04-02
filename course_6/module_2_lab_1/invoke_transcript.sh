#!/bin/bash

main_path=$PWD
echo $main_path
name="$1.wav"
wav_path="$main_path/wave/$name"
output_path="$main_path/transcript/$1.txt"
# please change the path to your own model
model_path="/home/project/modeltest/whisper/whisper.cpp/models/ggml-base.en.bin"
main_run="/home/project/modeltest/whisper/whisper.cpp/build/bin"
transcribe_audio_dir () {

	echo "Transcribing $wav_path"
    echo "model $model_path"
    echo "main $main_run"
	"$main_run/whisper-cli" -m "$model_path" -f "$wav_path" > "$output_path"

}
transcribe_audio_dir