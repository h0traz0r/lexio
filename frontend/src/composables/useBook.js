import { ref } from 'vue'

const book = ref(null) // { title: string, pages: string[] }

export function useBook() {
  function setBook(data) {
    book.value = data
  }

  function clearBook() {
    book.value = null
  }

  return { book, setBook, clearBook }
}
