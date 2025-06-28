(erase-messages)
(setq debug-on-error t)


(defun utf8-stream-function-to-macro-region (beg end)
  "."
  (interactive "*r")
  (let* ((regexp
          "\\(get_byte_at_index\\|get_byte_slice_of\\|get_str_slice_of\\)(\\([^)]+\\))")
         (replacement "dbg_\\1!(\\2)"))
    (save-mark-and-excursion
      (replace-regexp-within-bounds regexp replacement beg end))))


(defun utf8-stream-function-to-macro-buffer()
  (interactive)
  (let* ((beg (point-min))
         (end (point-max))
         (regexp
          "\\(get_byte_at_index\\|get_byte_slice_of\\|get_str_slice_of\\)(\\([^)]+\\))")
         (replacement "dbg_\\1!(\\2)"))
    (if mark-active
        (user-error "mark is active, use utf8-stream-function-to-macro-region instead.")
      (save-excursion
        (widen)
        (replace-regexp-within-bounds regexp replacement beg end)))))

(defun utf8-stream-macro-to-function-region (beg end)
  "."
  (interactive "*r")
  (let* ((regexp
          "dbg_\\(get_byte_at_index\\|get_byte_slice_of\\|get_str_slice_of\\)!(\\([^)]+\\))")
         (replacement "\\1(\\2)"))
    (save-mark-and-excursion
      (replace-regexp-within-bounds regexp replacement beg end))))


(defun utf8-stream-macro-to-function-buffer()
  (interactive)
  (let* ((beg (point-min))
         (end (point-max))
         (regexp
          "dbg_\\(get_byte_at_index\\|get_byte_slice_of\\|get_str_slice_of\\)!(\\([^)]+\\))")
         (replacement "\\1(\\2)"))
    (if mark-active
        (user-error "mark is active, use utf8-stream-function-to-macro-region instead.")
      (save-excursion
        (widen)
        (replace-regexp-within-bounds regexp replacement beg end)))))
