WITH table_1 AS
				(SELECT title,
						country,
						salary + payroll_tax + benefits_cost AS _expr_0,
						salary + payroll_tax AS _expr_1,
						salary
					FROM employees
					WHERE start_date > DATE '2021-01-01' )
SELECT title,
	country,
	AVG(salary),
	SUM(salary),
	AVG(_expr_1),
	SUM(_expr_1),
	AVG(_expr_0),
	SUM(_expr_0) AS sum_gross_cost,
	COUNT(*) AS ct
FROM table_1
WHERE _expr_0 > 0
GROUP BY title,
	country
HAVING COUNT(*) > 2000
ORDER BY sum_gross_cost,
	country DESC
LIMIT 20
