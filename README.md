#  colossal

## A Colossal Cave Adventure Remake 

This is a port of Will Crowther's original Colossal Cave Adventure based on Fortran source from March 31, 1977. As such, it lacks later additions made by Don Woods such as scoring, more treasures and locations, etc. that some players may be familiar with.

## Why?
Good question. An almost ridiculous number of ports and modifications of this game already exist, but I wanted this port to get as close as possible to the user experience of one of Crowther's very early versions. I hope this will help keep this amazing piece of computing history accessible to a larger audience than the original source would easily allow.

##Is this a good example of Rust code?
NO! It is meant to be similar to the original Fortran, which uses a mess of goto statements (something Rust doesn't have). Look upon it and be content that we don't have to write code like this anymore!

##How accurate is the game play to the original?
It is as accurate as I could make it. I have been unable to get the original to compile, so some behavior is conjecture on my part. Namely, input parsing seems to have trouble if the first word is less than four characters, e.g. "EAT FOOD" doesn't work, but "EAT  FOOD" does. I do not know if this is a bug in my code, or the original. Help in compiling the original would be appreciated.
