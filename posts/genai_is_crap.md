---
title: Machine Learning Isn't Magic
slug: ml_isnt_magic
published: 2024-07-07
tags: [machine-learning, ai, philosophy]
public: false
---

# Machine Learning Isn't Magic and LLMs are boring

See the title, end post.

## Ok but really

My masters was in machine learning, specifically Time Series
Machine Learning. It's a fascinating, and somewhat underexplored branch of
time series analysis.

Time Series Machine Learning applies to a vast array of
domains, ranging from Climate Modeling, Stock Markets, and my areas of interest,
Electroencephalography [^eeg-spelling] and Electrocardiography [^ecg-spelling].

[^eeg-spelling]: I totally didn't need to look up the spelling for this.
[^ecg-spelling]: Or this.

All that is to say, I find machine learning incredibly interesting, and am
somewhat of a dab hand with both it, and time series data in
general. Because of this background, people are often surprised when they find
out that I am not "excited"/"hyped"/"interested" in the current wave of """AI"""
tools that are rapidly proliferating.

The goal of this post is to provide a single source I can point to when I end up
in this discussion with people, as well as getting a bit of catharsis by
screaming all of this text into the void that is the internet.

## How Machine Learning Works

Machine Learning is conceptually simple. In semi-laymans terms we write code which
when presented with some input and a label, it "remembers" both, "attaching"
the label to the input. Then, when given an "unlabeled" input, it tries to find
the "closest" "remembered" input.

To make this example somewhat more concrete, we'll discuss the "Hello World" of
machine learning. Hand-written character recognition.

Given a bunch of photos of hand written numbers [0..9], and labels for these
images (Eg, an image, and a bit of data that says "This image contains the
number 3") we "train" a model (We make it "remember" these pairs). Now after
giving it a few hundred/thousand of these hand-labeled handwriting examples, we
can present the model with a new, entirley unlabeled photo of a hand written
number. The model will then try and find the "closest" image it has in its
"memory" and will confidently say "this is this number".

This process we discussed is generally known as "classification". The other form
of machine learning, and the one which has the most hype behind it is
"regression" or more broadly, "prediction".

### Regression Problems

You know how to do regression! It's one of those things you're taught in high
school maths, but usually with the term "find the line of best fit"!

One of the things you were also probably asked to do in the same maths lessions
you learned how to draw these lines of best fit, was to make predictions based
on that same line. This is using a linear regression to make linear predictions.

In essence, regression analysis is the act of using algorithms to find the line
of best fit programatically. There's a vast array of methods for finding these
lines, and just as many possible _types_ of line (Linear regression, polynomial
regression, binomial regression, logistic regression....).

Prediction then, is using the line we get from regression to make guesses as to
what the data would be at a given coordinate, just like you did in maths class!

What makes this interesting however, is how we can generalise regression
problems into higher dimensions. Our examples above focused on 2d plots, but we
can perform regressions on almost any amount of dimensions [^n-dim-slow]. This
is the foundational model upon which almost all of this ""generative ai"" exists.

[^n-dim-slow]: Of course it must be stated that the more dimensions we're working with the slower, and more complex this problem becomes

### How GenAI works for dummies

One of the neat things about maths is that it's often possible to reframe one
problem as another one. Classic examples of this are the reduction of
NP-Complete problems [^np-reduction]. Almost everything in machine learning can
be reduced into either a classification problem, or a regression problem, and in
fact LLMs as a whole are just one incredibly complex regression problem.

[^np-reduction]: for example, the travelling salesman problem can be, through a series of reductions through other NP-Complete problems eventually turned into a boolean satisfiability problem

An LLM is a statistical model. Given a text, what are the next
words that should be produced? Or to phrase it with our newly found knowledge;
Given some input, within this multi-billion-dimension regression problem, what
are the next set of values that should appear on this line?

This is what ChatGPT and other LLMs are doing. Your input is the start of a
line, and the machine is "guessing" based on it's training data what the
appropriate response is.

### The Chinese Room and what it means to be Sentient

In the seminal 1980 paper Minds, Brains and Programs the philosopher John Searle put
forward a thought experiment.
Assume that some AI has been developed which "understands" Mandarin. The machine
takes input in Mandarin, and produces an output also in Mandarin.
As a result of this rote translation by mechanical means, and an unfathomal
amount of computational power it is able to convince some people, that this is a
real machine which can read, and write in Mandarin. Truly an amazing piece of
technology!

The thought experiemnt then goes on to posit a further idea. If I decided to
lock you into a room [^mandarin-speakers], with nothing but a Mandarin to Mandarin phrasebook, paper,
pens, an english version of the "computer program" detailing the steps to "run
the program" with said materials, and an unlimited supply of Nanotrasen
SpacePaste to fulfil your dietary needs, you would be able to achieve the
same result as the machine.

This then raises an _incredibly_ interesting question. Do either of these
machines actually _understand_ Mandarin? The machine and yourself are producing
equally rote responses to a given input. You may learn eventually that x input
always results in y output, but without any frame of reference in a language you
already understand it is unlikeley you will ever have any clue what either the
input, or output you produce _means_.

[^mandarin-speakers]: Of course this analogy falls flat if you do speak and understand mandarin but in this case replace all instances of the word "you" with "Some random person"

Much like the idea of transforming the travelling salesman into a SAT problem,
we can also transform this understanding of the Chinese Room thought experiment
into an interesting philosophical razor for the alleged "intelligence" of LLMs.

An LLM has no way of actually _understanding_ what it's input means, nor what
it's actually outputting. All it sees are your inputs translated into a set of
numbers, and it responds with another set of numbers which in turn correspond to
a set of output symbols.
Because of their entirley statistical nature, LLMs are incapable of learning
anything new without training. LLMs are incapable of producing differing outputs
from the same inputs without baking in some randomness to the model, which also
means they will never produce the same output!

LLMs are philosophically interesting, but as someone who is keenly interested in
the cutting edge of machine learning, seeing an entire field reduced to a more
advanced version of Cleverbot is incredibly disappointing.

### Lies, More lies and Statistics

Whatever is at the current edge of discovery is always going to have grifters
trying to sell you snake oil. Patent medicine was often either useless or
downright toxic [^lillythepink].
Cryptocurrency and "blockchain" was usually just a veil over a
pump and dump or rugpull scheme.

[^lillythepink]: Obviously aside from Lilly The Pink's Medicinal Compoud which was most efficacious in every case.

Right now, there is huge amounts of money and FOMO-based advertising which
directly targets the management class of companies in order to sell them a
product which in almost all cases is only marginally useful at best.

The potential loss of "not being in on the ground floor" for whatever the next
revolutionary technology is, is often horrifying for these types of people, and
AI grifers are explicitly targeting this by making promises about the technology
of LLMs which are _just not possible_.

One of the most glaring examples of this in recent memory come from a lot of
"Thought Leaders" in the area saying that LLMs are "close to AGI" and "require
just a bit more training" which is complete, and utter nonsense. By their very
nature LLMs are incapable of reasoning, which is the defining feature of an
artificial general intelligence.

"AI" is the new buzzword. LLMs are trivially implementable, it's at most 50
lines of python to deploy an OpenAI powered LLM so you can outsource the work of
handling support requests to useless chatbots which have never helped anyone.

I predict the current wave of ""AI"" grifters promising LLMs as a "stepping
stone to general AI" are not going to be going away any time soon, and will
continue to try and sell the cult's kool-aid until some incident along the lines
of the FTX fallout happens to OpenAI and it's ilk.

## Summary

I don't like LLMs. I don't like the fact that "AI" research has been reduced to
an overdeveloped version of cleverbot. I don't like the fact that it's big tech
trying to create a hypergrowth market that doesn't and cant exist. I don't like
the fact that the web is now being polouted by a bunch of LLM generated Slop. I
don't know anyone who actually knows about machine learning that is interested
in any of this crap.


### Outro
- Music: [Untrue by Burial](https://www.youtube.com/watch?v=Os9DYRZyk-w)
- Coffee: LA SIGUANABA from Dark Arts still. Brewed with a V60, standard papers, 23 on
  Commandante C40
