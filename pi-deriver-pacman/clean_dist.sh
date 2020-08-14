#!/bin/bash

work_dir=$(pwd)

rm -rf ${work_dir}/dist/archlinuxfr/*
rm -rf ${work_dir}/dist/community/*
rm -rf ${work_dir}/dist/core/*
rm -rf ${work_dir}/dist/extra/*
rm -rf ${work_dir}/dist/multilib/*

echo Completed