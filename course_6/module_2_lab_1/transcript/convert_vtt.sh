#!/bin/bash

# Function to convert the format
convert_subtitle_format() {
    input_file="$1.txt"
    output_file="$1.vtt"

    if [ -e "$output_file" ]; then
        echo "File $output_file exists. Deleting it now..."
        rm "$output_file"
    fi
    echo "Converting: $input_file"
    count=1

    echo "WEBVTT" >> "$output_file"

    while IFS= read -r line; do
        # Extract timestamp and text using regex
        if [[ $line =~ \[([0-9:.]+)" --> "([0-9:.]+)\]\s*(.+) ]]; then
            start_time="${BASH_REMATCH[1]}"
            end_time="${BASH_REMATCH[2]}"
            text="${BASH_REMATCH[3]}"

            # Write converted format to the output file
            echo "$count" >> "$output_file"
            echo "$start_time --> $end_time" >> "$output_file"
            echo "$text" >> "$output_file"
            echo "" >> "$output_file"
            count=$((count + 1))
        fi
    done < "$input_file"

    echo "Converted subtitles saved to: $output_file"
}

# Check if the input file is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <input_txt_file>"
    exit 1
fi

# Call the function with the input file
convert_subtitle_format "$1"