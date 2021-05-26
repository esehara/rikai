
{{problem}}

let int_list_of_string s =
  String.split_on_char ' ' s
  |> List.map(int_of_string);;

{{limit}}

{{input}}

{{output}}


{% for example in examples %}
{{example.text}}
{% if example.pre_has %}
  {% if example.pre_is_oneline %} 
let example{{loop.index}} = "{{example.pre_oneline}}";;
  {% else %}
let example{{loop.index}} = [
    {% for pre in example.pre %}"{{pre}}";
    {% endfor %}
];;
  {% endif %}
{% endif %}
{% endfor %}