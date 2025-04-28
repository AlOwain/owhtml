# OwHTML
> A toy HTML parser made to learn how browsers are made.

> [!WARNING]
> This is built solely so I could learn more on web platforms, do with it as you wish; with that in mind.

## Long-term Goal

This project aims to provide a normalized HTML form. It will do the following pre-processing stages:
- It should remove stop words which includes
  - Accessibility information, such as alternative text and captions.
  - All text and images, only the length of the text and the size of the image would remain. (Should the resolution be kept as it might affect the size, or could it be represented in the style).
- Normalization should happen as a side-effect of parsing the HTML.
- We could stem redundant tags, such as headings, spans and divs, etc.
- Lemmatization would include removing color information, fonts, removing image size details, etc.

#### Why?

The purpose of this is to hold meaningful markup patterns, that would be intentionally limited in grammar and syntax. It is not meant to be human-readable, nor as a compression mechanism (as it is very lossy) but perhaps could be used to pre-process HTML thus make AI generation of Image -> HTML more efficient / better.

#### Reversal

While the process is meant to be lossy, it should be somewhat reversible given only an image of the HTML file, color information can be applied to the proper tags from the image, or a supplementary program could suggest color palettes (given only the normalized form of the HTML file); while missing text could not be sufficiently provided, OCR could be used to include the excluded text and font information, or placeholder text could be inserted, or even text suggested through an LLM, and finally alternative text and captions could be suggested by LLMs—I recognize that this is not ideal at all. preprocessor

#### Constraints

This sub-section defines the constraints this program is under, which are not many.
- This does not adhere to the HTML5 standard—not remotely—nor does it aim to. It will simply work for the exact purpose of pre-processing "HTML" into a simple grammar. It is written in the hopes that you wouldn't mess with it too much, and not give it improper HTML.
- The syntactic structure is well-defined in the HTML standard, so creating an AST is trivial.
- HTML has some dialectal variation, but it never overlaps nor contradicts, which makes parsing even simpler.
- HTML is sometimes inconsistent in ways that are tolerated by browsers, but HTML5 validators could identify inconsistencies pretty effectively.

### So what does it do now?

...It doesn't do much? I am very early in the process of parsing the HTML.

### To-do list:

- Create categorized test database.
- Move the to-do list into a KanBan table.
- Figure out how to encode styling; should there be a table of all styles for each element? It won't be so large, and there is a lot of space for improvement by pruning styles that don't effect the element, another method would be to create a key-value dictionary of applied styles.
- How should we resolve images; images sometimes come as background images, or just stylistic images, in Discord many SVG images are used stylistically in the background. The New York Times often use images as top banners, a simple wireframe of an image could be too simple.
- How should we handle invalid syntax;
  - We currently handle syntax errors by doing what's simplest. If it simpler to not handle improper syntax, then we don't and if it's simpler to ignore them like browsers, then we do.
  - Maybe we should actually ignore errors like browsers, it might be simpler than having to properly handle them, moreover, they are commonplace, so handling them could limit our datasets, and we could use HTML validators to confirm we are doing it correctly.
  - Properly handling them might minimize the divide between browsers and unexpected behavior, which would then be harder to normalize.