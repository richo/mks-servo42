mks_servo42
===========

A rust "driver" for the mks_servo series of closed loop servo motors.

... "driver" ??
---------------

driver is in air quotes because this is basically just a library for formatting
the commands. My assumption is that you're probably using this in a resource
constrained environment and rather than trying to bind you to any particular
Read and Write traits we'll just loan you a slice out of our buffer to send,
and provide a general purpose parser for returned data[^parser].

[^parser]: The parser is a pretty classic instance of README driven development
  we'll see if it happens.


