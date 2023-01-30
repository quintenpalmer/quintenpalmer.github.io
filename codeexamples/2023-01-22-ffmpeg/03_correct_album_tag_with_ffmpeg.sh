set -e

mv example_song.flac old_example_song.flac

ffmpeg \
	-i old_example_song.flac \
	-c copy \
	-metadata "ALBUM=Fun Songs for Dancing" \
	example_song.flac

