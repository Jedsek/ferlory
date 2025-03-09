#let template(body) = {
  show math.equation.where(block: false): it => html.elem("span", html.frame(it))
  show math.equation.where(block: true): html.frame
  show image: it => html.elem("img", attrs: (src: it.source))
  body
}

#show: template

// #set math.equation.content(fill: red)
#set text(
  fill: white,
  size: 2em,
  weight: "extrabold"
)

= Introdu
In this report, we will explore the
various factors that influence _fluid
dynamics_ in glaciers and how they
contribute to the formation and
behaviour of these natural structures.

Total displaced soil by glacial flow:

$ 7.32 beta +
  sum_(i=0)^nabla
    (Q_i (a_i - epsilon)) / 2 $

你好
->
