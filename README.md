# Rust Async Example

This is a toy example for asynchronous Rust code which uses the tokio runtime and
channels for communication. It is intended for learning purposes only and has no 
real world application.

## What does the Code do?

Using the tokio runtime a lot of asynchronous tasks are created which sleep for a random amount
of time and then return the time they have slept using a channel. This mimics the "goroutine" approach
of the Go programming language. 
