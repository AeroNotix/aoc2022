(defun find-most-calorifically-dense-elf-in-file (path n)
  (with-open-file (f path)
    (loop
      for line = (read-line f nil)

      with elf = 0
      with elves = nil

      while line do (progn
                      (let ((line (string-right-trim '(#\Newline) line)))
                        (if (string= "" line)
                            (progn
                              (push elf elves)
                              (setf elf 0))
                            (incf elf (parse-integer line)))))
      finally (return
                (apply #'+ (subseq (sort elves #'>) 0 n))))))

(defun main ()
  (mapcar (lambda (f)
            (format t "~A~%" (find-most-calorifically-dense-elf-in-file f 1))
            (format t "~A~%" (find-most-calorifically-dense-elf-in-file f 3)))
          (cdr sb-ext:*posix-argv*)))

(main)
