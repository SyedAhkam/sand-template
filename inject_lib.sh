#!/usr/bin/env bash

target=aarch64-linux-android
output_dir=android/jniLibs

cargo ndk -t $target -o $output_dir build
