ffmpeg \
	-f lavfi \
	-i "sine=frequency=220:duration=4" \
    -metadata ARTIST="The Cool Band" \
    -metadata ALBUM_ARTIST="The Cool Band" \
    -metadata ALBUM="Funky Songs for Dancing" \
    -metadata DISCNUMBER=1 \
    -metadata DISCTOTAL=2 \
    -metadata TRACK=1 \
    -metadata TRACKTOTAL=14 \
    -metadata TITLE="Wonderful Time" \
    -metadata DATE=2009 \
    -metadata GENRE="Dance Pop" \
    example_song.flac
