module.exports = grammar({
    name: '{{cookiecutter.language_slug}}',

    rules: {
        // TODO: add the actual grammar rules
        source_file: $ => 'hello',
    }
});
