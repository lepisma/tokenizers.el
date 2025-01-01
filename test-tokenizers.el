;;; Unstructured tests for tokenizers

(progn
  (add-to-list 'load-path default-directory)
  (require 'tokenizers)

  (setq tk (tokenizers-from-pretrained "sentence-transformers/all-MiniLM-L6-v2")))

(tokenizers-encode tk "Test sentence with some words")
