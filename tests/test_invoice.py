from python_rust_project_template import Invoice

def test_invoice():
    invoice = Invoice(2500, 0)
    assert invoice.total() == 2500.

def test_invoice_with_tax():
    invoice = Invoice(2500, 0.2)
    assert invoice.total() == 3000

def test_invoice_with_invalid_tax():
    try:
        Invoice(2500, 1.1)
        assert 1 == 0
    except ValueError as _:
        assert 1 == 1
