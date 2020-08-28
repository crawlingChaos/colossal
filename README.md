#  colossal

## A Colossal Cave Adventure Remake 

This is a port of Will Crowther's original Colossal Cave Adventure based on Fortran source from March 31, 1977. As such, it lacks later additions made by Don Woods such as scoring, more treasures and locations, etc. that some players may be familiar with.

## Why?
Good question. An almost ridiculous number of ports and modifications of this game already exist, but I wanted this port to get as close as possible to the user experience of one of Crowther's very early versions. I hope this will help keep this amazing piece of computing history accessible to a larger audience than the original source would easily allow.

## Is this a good example of Rust code?
NO! It is meant to be similar to the original Fortran, which uses a mess of goto statements (something Rust doesn't have). Look upon it and be content that we don't have to write code like this anymore!

## How accurate is the game play to the original?
It is as accurate as I could make it. I'd say it's almost exact, except for handling a couple bugs that crash the original game (noted in the source). I was able to compile and run the original source under a PDP-10 emulator and the output is identical to my observations. I also used a short transcript of Colossal Cave Adventure play on a PDP-10 as a guide for some line spacing, as different implementations of Fortran seem to vary in their handling of formatting issues.

## Changes
Version 1.0.0 has extensive changes from the previous version - due mostly to my negligence. My original code was based on Fortran 77 code I thought was Crowther's original, but turned out to be a slight rewrite by Matthew Russotto of the original Fortran IV source. Both sets of source were found in the same archive, so I had assumed both were Crowther's. This new version corrects this oversight, affecting both the appearance of the output and input handling. Only upper case input is recognized now. I also made some improvements to the fortran helper code, allowing the main program to retain a greater look and feel of the original fortran. I now consider this essentially complete, barring bugs or mistaken assumptions in my implementation.
