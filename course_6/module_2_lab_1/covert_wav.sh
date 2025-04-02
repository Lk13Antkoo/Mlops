#!/bin/bash

main_path=$PWD

echo $main_path
name="$1.mp4"
video_path="$main_path/video_folder/$name"
output_path="$main_path/wave/$1.wav"
echo $video_path
echo $output_path
cvt_videos () {
	ffmpeg -i "$video_path" -ar 16000 -ac 1 -c:a pcm_s16le "$output_path"
}
cvt_videos

