# rsume

Rsume is a command line tool to generate CVs from data on a json, yaml or toml file.

> [!WARNING]
> This tool is still in early development. Many features won't work right and 
> documentation is almost completely missing. Expect limited functionality and
> breaking changes. 


## Current work

- [ ] Finish basic template.
- [ ] Better user errors.
- [ ] Bug fixes.
- [ ] Code clean up.
- [ ] Documentation.

## Instalation

Right now, nothing is provided for an easy install, as the software is not stable.

You may clone this repo, then cargo build, then do with the executable as you like.


## Use

Validate your data:

```sh
rsume validate -i <cv.json>
```

Generate a pdf:

```sh
rsume generate -f pdf -i <cv.json> -t <path/to/template> -l <locale>
```

Live preview it in the browser:

```sh
rsume live -i <cv.json> -t <path/to/template> -l <locale>
```


## Contribution

TODO


## Internationalization

Some bits and pieces are in place, but only gregorian calendar dates are supported.


## Future work

If I had more time to work on this and could take it a step further I'd love to see:

- A way of registering templates with a name, perhaps even from a github repos.
- More or better ways to filter items when generating CVs.
- A less developer-focused web interface to edit your CV's data and produce the PDF


## Philosophy

I wanted a tool that would allow me to keep all my work experience, education, etc. 
in just one file, and filter it to generate one PDF document to submit to job postings.

This tool tries to balance reasonable flexibility, you should be able to include whatever 
you want in your CV, with sane models and schemas because I'm always wondering what to
call the different sections, which sections to include, etc.


## License

Fuck licenses, check `LICENSE.md` for more information.


## How do you read the name of this piece of software?

I sometimes read it like "résumé", other times like "arr-ess-you-me".


## Free Palestine
