#!/bin/sh

REL_DST=".experimental/release/"
REL_DST_LIB="${REL_DST}lib/"
REL_SRC=""
REL_SRC_LIB="buildroot/buildroot-2025.02.2/output/host/mipsel-buildroot-linux-musl/sysroot/"
REL_DST_FONTS="${REL_DST}fonts/"
SRC_LIBS="libxkbcommon.so.0
          libinput.so.10
          libfontconfig.so.1
          libudev.so.1
          libgcc_s.so.1
          libc.so
          libmtdev.so.1
          libevdev.so.2
          libfreetype.so.6
          libbz2.so.1.0
          libpng16.so.16
          libbrotlidec.so.1
          libbrotlicommon.so.1
          "



mkdir -p "$REL_DST" "$REL_DST_LIB"

cp target/mipsel-unknown-linux-musl/release/nanowave "$REL_DST"
cp scripts/nanowave.sh "$REL_DST"

for lib in $SRC_LIBS; do
  echo $lib
  if [ -f "${REL_SRC_LIB}usr/lib/$lib" ]; then
    cp "${REL_SRC_LIB}usr/lib/$lib" "$REL_DST_LIB"
  elif [ -f "${REL_SRC_LIB}lib/$lib" ]; then
    cp "${REL_SRC_LIB}lib/$lib" "$REL_DST_LIB"
  else
    echo "MISSING LIB: $lib"
  fi
done


mkdir -p "${REL_DST_FONTS}/conf.d/" "${REL_DST_FONTS}"
cp "${REL_SRC_LIB}etc/fonts/fonts.conf" "${REL_DST_FONTS}"

# cd fonts && ./build-cache.sh && cd -
cp -rp fonts/{cache,ttf} "${REL_DST_FONTS}/"

# buildroot/buildroot-2025.02.2/output/host/mipsel-buildroot-linux-musl/sysroot/usr/share/xml/fontconfig/fonts.dtd
# fc-cache -f -v fonts/ttf

sed -i \
  -e 's#<dir>/usr/share/fonts</dir>#<dir prefix="cwd">./ttf</dir>#' \
  -e 's#<dir>/usr/local/share/fonts</dir>##' \
  -e 's#<cachedir>/var/cache/fontconfig#<dir prefix="cwd">./cache</cachedir>#' \
  "${REL_DST_FONTS}fonts.conf"




#  && cp target/mipsel-unknown-linux-musl/release/nanowave "$REL_DST" \
#  && cp ${REL_SRC_LIB}usr/lib/libxkbcommon.so.0 "${REL_DST_LIB}" \
#  && cp ${REL_SRC_LIB}usr/lib/libinput.so.10 "${REL_DST_LIB}" \
#  && cp ${REL_SRC_LIB}usr/lib/libfontconfig.so.1 "${REL_DST_LIB}" \
#  && cp ${REL_SRC_LIB}lib/libudev.so.1 "${REL_DST_LIB}" \
#  && cp ${REL_SRC_LIB}lib/libgcc_s.so.1 "${REL_DST_LIB}" \
#  && cp ${REL_SRC_LIB}lib/libc.so "${REL_DST_LIB}"

# patchelf --set-interpreter '$ORIGIN/lib/libc.so' "${REL_DST}/nanowave"
# patchelf --set-rpath '$ORIGIN/lib' "${REL_DST}/nanowave"


patchelf --set-interpreter '/usr/data/mnt/sd_0/bin/lib/libc.so' "${REL_DST}/nanowave"
patchelf --set-rpath '/usr/data/mnt/sd_0/bin/lib' "${REL_DST}/nanowave"

if [ "$1" = "push" ]; then
  adb push .experimental/release/* /usr/data/mnt/sd_0/bin/
fi


# ./nanowave.sh
  #Error loading shared library libmtdev.so.1: No such file or directory (needed by /usr/data/mnt/sd_0/bin/lib/libinput.so.10)
  #Error loading shared library libevdev.so.2: No such file or directory (needed by /usr/data/mnt/sd_0/bin/lib/libinput.so.10)
  #Error loading shared library libfreetype.so.6: No such file or directory (needed by /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1)
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_set_device_log_function: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_enable_event_code: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_slot_value: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: mtdev_put_event: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_event_type_get_max: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_event_code_get_name: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_disable_property: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_abs_resolution: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_set_clock_id: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_abs_maximum: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_has_property: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: mtdev_new_open: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: mtdev_close_delete: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_disable_event_code: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_enable_event_type: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: mtdev_get_event: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_has_event_type: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_current_slot: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_event_type_get_name: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_free: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_abs_fuzz: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_set_abs_maximum: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_event_type_from_name: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_set_abs_resolution: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_change_fd: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_id_product: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_num_slots: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_abs_info: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_enable_property: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_set_abs_fuzz: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_event_value: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_has_event_code: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_property_from_name: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_event_is_code: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_fetch_slot_value: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_next_event: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_id_bustype: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_fd: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_disable_event_type: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_event_code_from_name: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_property_get_name: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_new_from_fd: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_fetch_event_value: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_id_vendor: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_get_name: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: libevdev_set_abs_info: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libinput.so.10: mtdev_empty: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Get_Postscript_Name: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Get_MM_Var: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Set_Var_Design_Coordinates: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_New_Face: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Get_First_Char: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Get_BDF_Property: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Get_X11_Font_Format: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Select_Size: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Get_Sfnt_Name_Count: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Done_FreeType: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Load_Sfnt_Table: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Set_Named_Instance: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Get_Next_Char: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Select_Charmap: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Get_Sfnt_Table: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Get_Advance: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Get_Char_Index: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Done_Face: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Load_Glyph: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Get_Sfnt_Name: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Done_MM_Var: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Init_FreeType: symbol not found
  #Error relocating /usr/data/mnt/sd_0/bin/lib/libfontconfig.so.1: FT_Get_PS_Font_Info: symbol not found
