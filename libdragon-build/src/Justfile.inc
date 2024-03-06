lb-finish-rom:
    @just lb-build-assets

    @if [ "$ELF_FILE" -nt "$ELF_FILE.stripped" -o ! -f "$ELF_FILE.stripped" ]; then \
        "$DEP_LIBDRAGON_SYS_N64_TOOLDIR"/n64sym "$ELF_FILE" "$ELF_FILE.sym"; \
        cp "$ELF_FILE" "$ELF_FILE.stripped"; \
        "$DEP_LIBDRAGON_SYS_TOOLCHAIN_BIN"strip "$ELF_FILE.stripped"; \
    fi

    @if [ -f "$ELF_FILE.dfs" ]; then \
        "$DEP_LIBDRAGON_SYS_N64_TOOLDIR"/n64tool -t "$GAME_NAME" \
            --toc -o "$ELF_FILE.z64" --align 256 "$ELF_FILE.stripped" --align 8 "$ELF_FILE.sym" --align 16 "$ELF_FILE.dfs"; \
    else \
        "$DEP_LIBDRAGON_SYS_N64_TOOLDIR"/n64tool -t "$GAME_NAME" \
            --toc -o "$ELF_FILE.z64" --align 256 "$ELF_FILE.stripped" --align 8 "$ELF_FILE.sym" --align 8; \
    fi

    @echo "ROM is in $ELF_FILE.z64"

lb-build-assets:
    #!/usr/bin/env bash
    set -euo pipefail
    fs=$ELF_FILE.fs

    # create filesystem dir
    if [ ! -d "$fs" ]; then
        mkdir -p "$fs"
    fi

    # compile assets
    # TODO need a way to specify custom arguments, perhaps in the project's build.rs script
    if [ -d assets ]; then
        # MKSPRITE
        for sprite in `find assets -name "*.png"` ; do 
            fn=$(basename "$sprite" .png)
            if [ "$sprite" -nt "$fs/$fn.sprite" -o ! -f "$fs/$fn.sprite" ]; then 
                echo Compiling $sprite
                if [ "$fn" = "n64brew" ]; then
                    "$DEP_LIBDRAGON_SYS_N64_TOOLDIR"/mksprite -c 0 -o "$fs" --format RGBA16 --tiles 32,32 "$sprite"
                elif [ "$fn" = "tiles" ]; then
                    "$DEP_LIBDRAGON_SYS_N64_TOOLDIR"/mksprite -c 0 -o "$fs" --format CI4 --tiles 32,32 "$sprite"
                else 
                    "$DEP_LIBDRAGON_SYS_N64_TOOLDIR"/mksprite -c 0 -o "$fs" "$sprite"
                fi
            fi
        done

        # MKFONT
        for font in `find assets -name "*.ttf"` ; do
            fn=$(basename "$font" .ttf)
            if [ "$font" -nt "$fs/$fn.font" -o ! -f "$fs/$fn.font" ]; then
                echo Compiling $font
                "$DEP_LIBDRAGON_SYS_N64_TOOLDIR"/mkfont -c 0 -o "$fs" "$font"
            fi
        done

        # AUDIOCONV (.wav)
        for wav in `find assets -name "*.wav"` ; do
            fn=$(basename "$wav" .wav)
            if [ "$wav" -nt "$fs/$fn.wav64" -o ! -f "$fs/$fn.wav64" ]; then
                echo Compiling $wav
                "$DEP_LIBDRAGON_SYS_N64_TOOLDIR"/audioconv64 --wav-compress 3 -v -o "$fs" "$wav"
            fi
        done

        # AUDIOCONV (.xm)
        for xm in `find assets -name "*.xm"` ; do
            fn=$(basename "$xm" .xm)
            if [ "$xm" -nt "$fs/$fn.xm64" -o ! -f "$fs/$fn.xm64" ]; then
                echo Compiling $xm
                "$DEP_LIBDRAGON_SYS_N64_TOOLDIR"/audioconv64 -v -o "$fs" "$xm"
            fi
        done

        # AUDIOCONV (.ym)
        for ym in `find assets -name "*.ym"` ; do
            fn=$(basename "$ym" .ym)
            if [ "$ym" -nt "$fs/$fn.ym64" -o ! -f "$fs/$fn.ym64" ]; then
                echo Compiling $ym
                "$DEP_LIBDRAGON_SYS_N64_TOOLDIR"/audioconv64 --ym-compress false -v -o "$fs" "$ym"
            fi
        done
    fi

    # copy over files in ./filesystem
    if [ -d filesystem ]; then
        for fsfile in `find ./filesystem/ -type f` ; do
            fn=$(basename "$fsfile")
            assetdir=$(dirname "$fsfile" | sed 's,\./filesystem,,')
            if [ "$fsfile" -nt "$fs/$assetdir/$fn" -o ! -f "$fs/$assetdir/$fn" ]; then
                mkdir -p "$fs/$assetdir"
                cp -v "$fsfile" "$fs/$assetdir/$fn"
            fi
        done
    fi

    # make dfs
    lst=$(ls -A "$fs")
    if [ -n "$lst" -a "$fs" -nt "$ELF_FILE.dfs" ]; then
        "$DEP_LIBDRAGON_SYS_N64_TOOLDIR"/mkdfs "$ELF_FILE.dfs" "$fs" >/dev/null
    fi

    #echo "$fs"

