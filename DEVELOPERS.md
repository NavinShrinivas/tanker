# rrcon : The Redis consistency layer

This file documents all the things that are considered relevant research for this project.

## Redis data types

One of the first things to understand before writing a log translation layer is to understand the types of data that can be stored in a Redis database.

Use this to learn about this section : https://redis.io/docs/data-types/tutorial/

> Note: eventually we need to start restricting unsafe commands from propgating in the commands, something that wont lead to consistency in the network

Here are some of important notes wrt to this project : 

- `GETSET` operations should not be allowed are Rafts consistency is weakly strong.
