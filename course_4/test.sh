#!/bin/bash

# Default values
num=1
reverse=false
truncate_flag=false  # Renamed to avoid conflict with the `truncate` command
capitalize=false
deliminater=" "

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --number)
            num="$2"
            shift 2
            ;;
        -r)
            reverse=true
            shift
            ;;
        --truncate)
            truncate_flag=true
            shift
            ;;
        --capitalize)
            capitalize=true
            shift
            ;;
        --deliminater)
            deliminater="$2"
            shift 2
	    break
            ;;
    esac
done

phrase="$1"


# Output parsed values
echo "number: $num"
echo "reverse: $reverse"
echo "truncate_flag: $truncate_flag"
echo "capitalize: $capitalize"
echo "deliminater: '$deliminater'"
echo "phrase: '$phrase'"
