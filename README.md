# OwHTML
> A toy HTML parser made to learn how browsers are made.

As of the commit one-after e42ede1c, it basically just prints the so-called "AST".

> [!WARNING]
> This is built solely so I could learn more on browsers, do with it as you wish with that in mind.

### TODO list:

- Remove the tag name from the attributes while parsing.
- Check that `parse_handler` is implemented correctly. I worked on it while very tired.
- Check if recursive elements would work.
- Create basic regression tests.
.. Beyond this point are things to be done in the medium-term
- Research what to paint with:
  - Apparently a naïve pixel-buffer isn't the right thing to use, however, I do not want
  to bother with learning something complicated when I can barely draw text.
- Styling:
  - Create naïve text layout position
  - Style text (color, italics, ..)
  - Style text position (centered, right, ..)
- Embed images
- Understand how to want to handle errors, consider being syntactically strict
