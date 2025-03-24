# OwHTML
> A toy HTML parser made to learn how browsers are made.

> [!WARNING]
> This is built solely so I could learn more on web platforms, do with it as you wish; with that in mind.

### Long-term Goal

This project aims to provide a normalized HTML form. It will do the following pre-processing stages:
- It should remove stop words which includes
  - Accessibility information, such as alternative text and captions.
  - All text and images, only the length of the text and the size of the image would remain. (Should the resolution be kept as it might affect the size, or could it be represented in the style).
- Normalization should happen as a side-effect of parsing the HTML.
- We could stem redundant tags, such as headings, spans and divs, et cetera.
- Lemmatization would include removing color information, fonts, removing image size details, et cetera.

#### Reversal

While the process is meant to be lossy, it should be somewhat reversable given only an image of the HTML file, color information can be applied to the proper tags, alternative text and captions could be generated, OCR could be used to include the missing text and font information.

#### Why?

The purpose of this is to hold meaningful markup patterns, that would be intentionally limited in grammar and syntax. It is not meant to be human-readable, nor as a compression mechanism (as it is very lossy) but perhaps could be used to pre-process HTML thus make AI generation of Image -> HTML more efficient / better.

#### Constraints

This sub-section defines the constraints this program is under, which are not many.
- This does not adhere to the HTML5 standard—not remotely—nor does it aim to. It will simply work for the exact purpose of pre-processing "HTML" into a simple grammar. It is written in the hopes that you wouldn't mess with it too much, and not give it improper HTML.
- The syntactic structure is well-defined in the HTML standard, so creating an AST is trivial.
- HTML has some dialectal variation exists, but it never overlaps or contradicts, which makes parsing even simpler.
- HTML is sometimes inconsistent in ways that are tolerated by browsers, but HTML5 verifiers could identify inconsistencies pretty effectively.

### So what does it do now?

..It doesn't do much? I am very early in the process of creating a very basic DOM.

### TODO list:

- Styling:
  - Figure out how to encode styling; should there be a table of all styles
  for each element? It won't be as large as I imagine, and there is a lot of
  space for improvement by pruning styles that don't effect the element.
- Resolve images resolution.
- Create an SQLite database for tests, create boolean flags for each category
- Create a Kanban table for TODOs

### Miscellaneous

#### Invalid Syntax Handling

> [!FIXME]
> Work on invalid syntax philosphy:
> -   We basically handle syntax errors by doing what's simplest.
>   If it simpler to not handle improper syntax, then we don't
>   and if it's simpler to be as lenient as browsers, then we
>   just ignore the errors as browsers do.
> -   We sometimes have dealt with improper errors, because they
>   we initially did, how should we deal with other errors?
> -   Maybe we should actually ignore errors like browsers, it
>   might be simpler than having to properly report them.
> -   Dealing with errors might be easier because we can use HTML
>   verifiers to confirm we are doing it correctly.
> -   Ignoring them could be more useful, as they are so commonplace.

#### Recurrant Bugs

This section includes bugs that are frequently found, bugs that we are, by design, prone to:
- Skipping characters; we often skip characters as the parser is just one large iterator.
