(* ---- 最頻出関数 ---*)
let int_list_of_string s = 
  String.split_on_char ' ' s
  |> List.map(int_of_string);;

let string_1_list_of_string x =
  String.to_seq x
  |> List.of_seq
  |> List.map (String.make 1);;

  (* ----------------- *)

{{problem}}

(* テンプレートが吐き出された時間 *)
(*  {{time}} *)
(* 解いた時間: _時 _分*)
(* かかった時間: _分 *)

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