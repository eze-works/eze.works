+++
[
    title: "Time for Computers",
    date: ~D(2023-11-04),
    labels: ["musings"],
]
+++
The basic unit for CPU time is a nanosecond. This is too small a measure to develop an intuition for. But you can build intuition by comparison. In the "Time for Computers" episode of the [Two's Complement](https://www.twoscomplement.org/) podcast, Matt Godbolt tries to tackle this by comparing the CPU time scale with a human time scale. This really helped me, so i'm reproducing a hand-wavy transcription of that scale here:

The basic unit of work a CPU can do is an instruction cycle. During the cycle the CPU will fetch an instruction, decode it and execute it. 

CPUs take 1 cycle to do elementary operations like addition, subtraction, XOR's and such. A CPU cycle takes a third of a nanosecond. Charitably, we'll say a human doing the same kind of operation would take 1 second. 

So that's our scale:  _1/3 nanosecond for a CPU is like  1 second for a homo sapiens_

The next type of operation a CPU does is __multiplying__. It takes anywhere between 4 and 6 cycles for a CPU to do multiplication. That works out as ~1.3 nanoseconds, which in human time is 4 seconds. Now that's still plausible. You could imagine someone who is good at mental math taking about that long to compute 398 times 16 (...i am not good at mental math).

Next up is __division__ If you don't have some sort of look up table, you will need to use pencil and paper to manually calculate. That intuition is about right for CPUs. They can't do division that much better than humans can. Integer division is anywhere between 30 - 100 cycles (10 - 33 ns), which in our scale is anywhere between 30 seconds to a minute and a half of human time. This makes sense if you imagine the CPU needing to take out a pencil and paper.

CPUs read from memory as well. We are told that memory is slow, which is why we have CPU caches that are supposed to make things go faster. 

An access to __L1 cache__ is the fastest thing you can get. It's a tiny cache right next to the CPU on the order of 32K bytes in size. It takes 3 CPU cycles to read from L1, which is ~3 seconds in human terms. That's a bit like retrieving information from the sticky-note on your desk. 

__L2 cache__ is a bigger, further away cache. If we were comparing L1 to a sticky-note, L2 is like a set of ring-binders you have on the shelves behind you.  Accessing L2 is 10 cycles away, which in human terms would be 10 seconds away. Seems a bit quick for a human, but it's within the realm of possibility.

__L3__ is the final cache layer shared between CPUs. That takes about ~40 cycles to get information, which is 40 seconds in human time. 

Now if have to hit __main memory__, we are talking 100 - 120 nanoseconds. That is 6 minutes of human time. A trip down the elevator to the archives to get the book you need and go back up the elevator and back up to the office to put it in cache. In the working life of a computer whose working job is adding numbers together, that's twiddling your thumbs or taking a tea break. That's why all these performance geeks hate missing their cache so much

Now for the scary part. Reading from an __SSD__ takes about 50 microseconds (not nano anymore!)...which is 2 whole days of human time. That's ordering something on amazon.com whenever you ask your CPU to read a file from disk.  Disgusting. I will never use disk again /s.

If you are using a spinning disk whose head  is not positioned in the right place and has to seek to the right sector, we are talking 1 - 10 milliseconds...which is 1 - 12 months in human months...

At the far end of this scale is rebooting the computer. Assuming it takes 5 minutes to do so (plausible depending on your computer), that's 32 millennia in human years. A civilization-ending event for your CPU. Think again before rebooting your computers.
