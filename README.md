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

- Remove the tag name from the attributes while parsing.
- Check that `parse_handler` is implemented correctly. I worked on it while very tired.
- Check if recursive elements would work.
- Create basic regression tests.
.. Beyond this point are things to be done in the medium-term
- Styling:
  - Create naïve text layout position
  - Style text (color, italics, ..)
  - Style text position (centered, right, ..)
- Resolve images resolution.
- Understand how we want to handle errors, consider being syntactically strict
- Create an SQLite database for tests, create boolean flags for each category
- Create a Kanban table for TODOs
