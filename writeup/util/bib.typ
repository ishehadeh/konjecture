#let bib-added = state("bib-added", false)
#let bib = () => {
  context {
    if not bib-added.get() {
      bibliography("../main.bib", style: "ieee")
      bib-added.update(true)
    } else {

    }
  }
}