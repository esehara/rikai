
{{problem}}

{{limit}}

{{input}}

{{output}}


{% for example in examples %}
{{example.text}}
let example{{loop.index}} = [
{% for pre in example.pre %}  "{{pre}}";
{% endfor %}
];;
{% endfor %}