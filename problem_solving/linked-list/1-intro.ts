class LinkedListNode {
  public value: number;
  public next: LinkedListNode | null;

  constructor(value: number) {
    this.value = value;
    this.next = null;
  }
}

class LinkedList {
  public head: LinkedListNode | null;
  private size: number;

  constructor() {
    this.head = null;
    this.size = 0;
  }

  isEmpty() {
    return this.size === 0;
  }

  getSize() {
    return this.size;
  }

  prepend(value: number) {
    const newNode = new LinkedListNode(value);

    if (this.head) {
      newNode.next = this.head;
      this.head = newNode;
    } else {
      this.head = newNode;
    }
    this.size++;
  }

  getAllValues() {
    let values = "";
    let pointer = this.head;

    while (pointer) {
      values += `${pointer.value} `;
      pointer = pointer.next;
    }

    console.log(values);
  }
}

const linkedList = new LinkedList();
linkedList.prepend(10);
linkedList.prepend(20);
linkedList.prepend(30);
linkedList.prepend(40);
linkedList.getAllValues();
