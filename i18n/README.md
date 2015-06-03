# Expand patterns like "i18n".

## Problem

Given a pattern and a list of words, return all of the words that match
the pattern. The pattern will consist of ASCII letters and/or digits.
Letters must be matched exactly (ignoring case) while digits must be
composed into a number and that number of (any) characters must be matched.

The objective is to make the matching algorithm efficient when calculating
many matches so the cost of pre-processing the dictionary can be discounted.

For example:

    match("i18n", <english dictionary>) =>
      institutionalization
      intercrystallization
      interdifferentiation
      internationalization
    
    match("24", <english dictionary>) =>
      formaldehydesulphoxylate
      pathologicopsychological
      scientificophilosophical
      tetraiodophenolphthalein
      thyroparathyroidectomize

    match("cat", <english dictionary>) =>
      Cat
      cat

## Notes

I went to town on this problem because I wasn't sure what the best approach was:

* "naive": Converts the pattern to a regular expression and sequentially scans
  a word list. The implementation is pretty straight-forward.
* "sets": Coverts the word dictionary into a mapping of
  (*\<char\>*, *\<index of char\>*, *\<length\>*) => *\<set of words\>*
  so matching becomes a series of set intersections.

  For example, the pattern "i18n" would match:

  mapping[('i', 0, 20)] ∩ mapping[('n', 19, 20)].

  The implementation is understandable and has good performance when at
  least one intermediate set is small e.g. "i18n" leads to a single
  intersection operation between a length 23 set and a length 11 set.

* "prefix": Coverts the word dictionary into a mapping of
  *\<length\>* => *\<prefix tree\>* so matching involves maintaining
  a work queue of prefix trees to traverse in parallel.

  The implementation is somewhat difficult to understand but has
  good performance when the number of characters skipped is small or
  the prefix tree is sparse where there any many skipped characters e.g.
  "c1t", "internation8n".

## Source:

http://www.careercup.com/page?pid=google-interview-questions
