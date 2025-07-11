module.exports = {
	printWidth: 120,
	//tabWidth: 4,
	useTabs: true,
	semi: true,
	singleQuote: false,
	quoteProps: "as-needed",
	jsxSingleQuote: false,
	trailingComma: "none",
	bracketSpacing: false,
	jsxBracketSameLine: false,
	arrowParens: "always",
	endOfLine: "lf" // Apparently, the GitHub repository still uses CRLF. I don't know how to force it to use LF, and until someone figures that out, I'm changing this to auto because I don't want more than one line ending commit.
};
