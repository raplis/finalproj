import sys
import json
from transformers import pipeline

def generate_text(text, max_length=50):
    try:
        generator = pipeline('text-generation', model='EleutherAI/gpt-neo-125M')
        results = generator(text, max_length=max_length, num_return_sequences=1, truncation=True)
        return results
    except Exception as e:
        return {"error": str(e)}

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python app.py <input_text> [max_length]")
        sys.exit(1)

    input_text = sys.argv[1]
    max_length = int(sys.argv[2]) if len(sys.argv) > 2 else 50

    generated_results = generate_text(input_text, max_length)
    print(json.dumps(generated_results))
