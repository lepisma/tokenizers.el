;;; tokenizers.el --- Huggingface tokenizers for Emacs Lisp -*- lexical-binding: t; -*-

;; Copyright (c) 2025 Abhinav Tushar

;; Author: Abhinav Tushar <abhinav@lepisma.xyz>
;; Version: 0.1.2
;; Package-Requires: ((emacs "29"))
;; Keywords: tools, nlp, ml
;; URL: https://github.com/lepisma/tokenizers.el

;;; Commentary:

;; Huggingface tokenizers for Emacs Lisp
;; This file is not a part of GNU Emacs.

;;; License:

;; This program is free software: you can redistribute it and/or modify
;; it under the terms of the GNU General Public License as published by
;; the Free Software Foundation, either version 3 of the License, or
;; (at your option) any later version.

;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
;; GNU General Public License for more details.

;; You should have received a copy of the GNU General Public License
;; along with this program. If not, see <https://www.gnu.org/licenses/>.

;;; Code:

(require 'cl-lib)

(cl-eval-when (load eval)
  (unless (require 'tokenizers-core nil t)
    (if (or noninteractive
            (yes-or-no-p "Module tokenizers-core must be built.  Do so now? "))
        (let ((default-directory (file-name-directory (or load-file-name
                                                          buffer-file-name)))
              (build-command "make release"))

          (message "Building tokenizers-core module with %S" build-command)
          (with-temp-buffer
            (unless (zerop (call-process-shell-command build-command nil t t))
              (error "Failed to compile module tokenizers-core: %s" (buffer-substring-no-properties (point-min) (point-max)))))
          (message "Loading tokenizers-core")
          (require 'tokenizers-core))
      (user-error "Abort compilation for tokenizers-core"))))

(defalias 'tokenizers-from-pretrained #'tokenizers-core-from-pretrained)
(defalias 'tokenizers-enable-padding #'tokenizers-core-enable-padding)
(defalias 'tokenizers-encode #'tokenizers-core-encode)
(defalias 'tokenizers-encode-batch #'tokenizers-core-encode-batch)

(provide 'tokenizers)

;;; tokenizers.el ends here
