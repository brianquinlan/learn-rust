# Expand patterns like "i18n".

## Problem

Given a pattern and a list of words, return all of the words that match
the pattern. The pattern will consist of ASCII letters and/or digits.
Letters must be matched exactly (ignoring case) while digits must be
composed into a number and that number of (any) characters must be matched.

The objective is to make the matching algorithm efficient when calculating
many matches so the cost of pre-processing the dictionary can be discounted.

For example:

    match("i18n", <english dictionary>") =>
      institutionalization
	    intercrystallization
	    interdifferentiation
	    internationalization
    
	  match("24", <english dictionary>") =>
	    formaldehydesulphoxylate
	    pathologicopsychological
	    scientificophilosophical
	    tetraiodophenolphthalein
	    thyroparathyroidectomize

	  match("cat", <english dictionary>") =>
	    Cat
	    cat

## Source:

Unknown
