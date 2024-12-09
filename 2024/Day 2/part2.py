def read_number_sequences(filename):
    sequences = []
    
    with open(filename, 'r') as file:
        for line in file:
            sequence = [int(num) for num in line.strip().split(';')]
            sequences.append(sequence)
    
    return sequences

def is_sequence_safe(sequence):
    if len(sequence) < 2:
        return True
    
    differences = []
    for i in range(len(sequence) - 1):
        diff = sequence[i + 1] - sequence[i]
        differences.append(diff)

    all_increasing = all(diff > 0 for diff in differences)
    all_decreasing = all(diff < 0 for diff in differences)
    
    if not (all_increasing or all_decreasing):
        return False
    
    if all_increasing:
        return all(1 <= diff <= 3 for diff in differences)
    else:
        return all(-3 <= diff <= -1 for diff in differences)

def is_sequence_safe_with_dampener(sequence):
    if is_sequence_safe(sequence):
        return True
    
    for i in range(len(sequence)):
        dampened_sequence = sequence[:i] + sequence[i+1:]
        if is_sequence_safe(dampened_sequence):
            return True
    
    return False

def main():
    sequences = read_number_sequences('Day 2\data.csv')

    safe_count = sum(1 for sequence in sequences if is_sequence_safe_with_dampener(sequence))
    
    print(f"Total number of safe sequences with Problem Dampener: {safe_count}")

if __name__ == "__main__":
    main()