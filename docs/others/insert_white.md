# **Function: insert_white_spaces**

```python
def insert_white_spaces(sequence, k) -> str:
```

### **Description**
Inserts a white space every `k` residue.

### **Parameters**

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `sequence` | `str` | *Required* | String in which to insert the white spaces |
| `k` | `int` | *Required* | Number of characters to skip before inserting a white space |


### **Returns**

| Type | Description |
| :--- | :--- |
| `str` | Returns a new `str` with white spaces inserted. |


### **Examples**

```python
import dna_parser as dps

sequence= "attgtatagctagatgctg"
dps.insert_white_spaces(sequence, 3)

#Output:
# 'att gta tag cta gat gct g'
```