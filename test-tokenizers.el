;;; Unstructured interactive tests for tokenizers.el

(progn
  (add-to-list 'load-path default-directory)
  (require 'tokenizers))

(ert-deftest encode-like-sentence-transformers ()
  (let ((tk (tokenizers-from-pretrained "sentence-transformers/all-MiniLM-L6-v2")))
    (tokenizers-enable-padding tk 0 "[PAD]")
    (should (equal (tokenizers-encode tk "This is an example sentence" t)
                   (list [101 2023 2003 2019 2742 6251 102]
                         [0 0 0 0 0 0 0]
                         [1 1 1 1 1 1 1])))
    (should (equal (tokenizers-encode-batch tk ["This is an example sentence" "Each sentence is converted"] t)
                   (list [[101 2023 2003 2019 2742 6251 102]
                          [101 2169 6251 2003 4991 102  0  ]]
                         [[0 0 0 0 0 0 0]
                          [0 0 0 0 0 0 0]]
                         [[1 1 1 1 1 1 1]
                          [1 1 1 1 1 1 0]])))))
