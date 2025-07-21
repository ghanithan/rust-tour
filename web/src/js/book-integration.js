export class BookIntegration {
  constructor() {
    this.bookCache = new Map();
    this.baseUrl = 'https://doc.rust-lang.org/book/';
  }

  init() {
    console.log('Book integration initialized');
  }

  async getChapterContent(chapterRef) {
    if (this.bookCache.has(chapterRef)) {
      return this.bookCache.get(chapterRef);
    }

    try {
      const url = `${this.baseUrl}ch${chapterRef}.html`;
      const response = await fetch(`/api/book/${chapterRef}`);
      
      if (response.ok) {
        const data = await response.json();
        this.bookCache.set(chapterRef, data);
        return data;
      }
      
      return { url, chapter: chapterRef };
    } catch (error) {
      console.error('Failed to fetch book content:', error);
      return null;
    }
  }

  generateBookLinks(rustBookRefs) {
    if (!rustBookRefs || !rustBookRefs.specific_sections) {
      return [];
    }

    return rustBookRefs.specific_sections.map(section => ({
      title: section.title,
      chapter: section.chapter,
      url: section.url,
      relevance: section.relevance,
      description: this.getChapterDescription(section.chapter)
    }));
  }

  getChapterDescription(chapterRef) {
    const descriptions = {
      '1.1': 'Installation and setup of Rust development environment',
      '1.2': 'Writing and running your first Rust program',
      '1.3': 'Understanding Cargo, Rust\'s build system and package manager',
      '2': 'Building a complete guessing game to learn Rust basics',
      '3.1': 'Variables, mutability, and constants in Rust',
      '3.2': 'Rust\'s type system including scalars and compounds',
      '3.3': 'Function definitions, parameters, and return values',
      '3.4': 'Comments and documentation in Rust code',
      '3.5': 'Control flow with if expressions and loops',
      '4.1': 'Understanding ownership, Rust\'s key differentiating feature',
      '4.2': 'References and borrowing for flexible memory management',
      '4.3': 'String slices and other slice types',
      '5.1': 'Defining and instantiating structs',
      '5.2': 'Using structs in example programs',
      '5.3': 'Method syntax for structs',
      '6.1': 'Defining enums and their variants',
      '6.2': 'The match control flow operator',
      '6.3': 'Concise control flow with if let',
      '7.1': 'Packages and crates for organizing code',
      '7.2': 'Modules for controlling scope and privacy',
      '7.3': 'Paths for referring to items in the module tree',
      '7.4': 'Bringing paths into scope with use',
      '7.5': 'Separating modules into different files',
      '8.1': 'Vectors for storing lists of values',
      '8.2': 'Strings for storing UTF-8 encoded text',
      '8.3': 'Hash maps for storing key-value pairs',
      '9.1': 'Unrecoverable errors with panic!',
      '9.2': 'Recoverable errors with Result',
      '9.3': 'To panic! or not to panic!',
      '10.1': 'Generic data types for code reuse',
      '10.2': 'Traits for defining shared behavior',
      '10.3': 'Validating references with lifetimes'
    };

    return descriptions[chapterRef] || `Chapter ${chapterRef} content`;
  }

  getConceptExplanation(concept) {
    const explanations = {
      'ownership': {
        title: 'Ownership',
        summary: 'Rust\'s system for managing memory safety without garbage collection',
        keyPoints: [
          'Each value has a single owner',
          'When the owner goes out of scope, the value is dropped',
          'Ownership can be moved between variables'
        ],
        bookChapter: '4.1'
      },
      'borrowing': {
        title: 'Borrowing',
        summary: 'Using references to access data without taking ownership',
        keyPoints: [
          'References allow you to use values without taking ownership',
          'References are immutable by default',
          'You can have multiple immutable references or one mutable reference'
        ],
        bookChapter: '4.2'
      },
      'lifetimes': {
        title: 'Lifetimes',
        summary: 'Ensuring references are valid for as long as needed',
        keyPoints: [
          'Every reference has a lifetime',
          'Lifetime annotations describe relationships between references',
          'The borrow checker uses lifetimes to ensure memory safety'
        ],
        bookChapter: '10.3'
      },
      'variables': {
        title: 'Variables and Mutability',
        summary: 'How Rust handles variable binding and mutability',
        keyPoints: [
          'Variables are immutable by default',
          'Use mut to make variables mutable',
          'Constants are always immutable and computed at compile time'
        ],
        bookChapter: '3.1'
      },
      'functions': {
        title: 'Functions',
        summary: 'Defining and calling functions in Rust',
        keyPoints: [
          'Functions are defined with the fn keyword',
          'Parameters must have type annotations',
          'Functions can return values with or without the return keyword'
        ],
        bookChapter: '3.3'
      },
      'control-flow': {
        title: 'Control Flow',
        summary: 'if expressions, loops, and other control flow constructs',
        keyPoints: [
          'if is an expression that returns a value',
          'Rust has loop, while, and for loop constructs',
          'break and continue work with loops'
        ],
        bookChapter: '3.5'
      },
      'structs': {
        title: 'Structs',
        summary: 'Custom data types that group related values',
        keyPoints: [
          'Structs group related data together',
          'Fields can be accessed with dot notation',
          'Methods can be defined for structs'
        ],
        bookChapter: '5.1'
      },
      'enums': {
        title: 'Enums',
        summary: 'Types that can be one of several variants',
        keyPoints: [
          'Enums define types with multiple possible values',
          'Variants can hold different types and amounts of data',
          'match expressions work perfectly with enums'
        ],
        bookChapter: '6.1'
      },
      'error-handling': {
        title: 'Error Handling',
        summary: 'Rust\'s approach to handling recoverable and unrecoverable errors',
        keyPoints: [
          'panic! for unrecoverable errors',
          'Result<T, E> for recoverable errors',
          'Propagating errors with the ? operator'
        ],
        bookChapter: '9'
      },
      'collections': {
        title: 'Collections',
        summary: 'Data structures that can contain multiple values',
        keyPoints: [
          'Vec<T> for growable arrays',
          'String for UTF-8 text',
          'HashMap<K, V> for key-value storage'
        ],
        bookChapter: '8'
      }
    };

    return explanations[concept] || null;
  }

  createConceptCard(concept) {
    const explanation = this.getConceptExplanation(concept);
    if (!explanation) return null;

    return {
      title: explanation.title,
      summary: explanation.summary,
      keyPoints: explanation.keyPoints,
      bookLink: `${this.baseUrl}ch${explanation.bookChapter}.html`,
      bookChapter: explanation.bookChapter
    };
  }

  getRelatedConcepts(mainConcept) {
    const relationships = {
      'ownership': ['borrowing', 'lifetimes', 'variables'],
      'borrowing': ['ownership', 'lifetimes', 'references'],
      'lifetimes': ['ownership', 'borrowing', 'functions'],
      'variables': ['ownership', 'mutability', 'constants'],
      'functions': ['lifetimes', 'ownership', 'control-flow'],
      'structs': ['ownership', 'methods', 'traits'],
      'enums': ['pattern-matching', 'option', 'result'],
      'error-handling': ['result', 'option', 'panic'],
      'collections': ['ownership', 'borrowing', 'iterators']
    };

    return relationships[mainConcept] || [];
  }

  generateStudyPlan(currentConcepts, userProgress) {
    const conceptOrder = [
      'variables',
      'functions', 
      'control-flow',
      'ownership',
      'borrowing',
      'structs',
      'enums',
      'collections',
      'error-handling',
      'lifetimes'
    ];

    const plan = [];
    
    for (const concept of conceptOrder) {
      if (currentConcepts.includes(concept)) {
        const explanation = this.getConceptExplanation(concept);
        const relatedConcepts = this.getRelatedConcepts(concept);
        
        plan.push({
          concept,
          explanation,
          relatedConcepts,
          priority: currentConcepts.includes(concept) ? 'high' : 'medium',
          bookChapter: explanation?.bookChapter
        });
      }
    }

    return plan;
  }

  trackBookUsage(chapterRef, action = 'view') {
    // Track book chapter usage for analytics
    const usage = {
      chapter: chapterRef,
      action,
      timestamp: new Date().toISOString(),
      sessionId: this.getSessionId()
    };

    // Store in local storage for now
    const usageHistory = JSON.parse(localStorage.getItem('bookUsage') || '[]');
    usageHistory.push(usage);
    
    // Keep only last 100 entries
    if (usageHistory.length > 100) {
      usageHistory.shift();
    }
    
    localStorage.setItem('bookUsage', JSON.stringify(usageHistory));
    
    console.log('Book usage tracked:', usage);
  }

  getSessionId() {
    return sessionStorage.getItem('sessionId') || 
           `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  generateBookmarkableUrl(chapterRef, exerciseId) {
    const baseUrl = window.location.origin;
    return `${baseUrl}?exercise=${exerciseId}&book=${chapterRef}`;
  }

  async searchBookContent(query) {
    // In a real implementation, this would search through book content
    // For now, return mock search results
    const mockResults = [
      {
        chapter: '4.1',
        title: 'What is Ownership?',
        snippet: 'Ownership is Rust\'s most unique feature...',
        relevance: 0.95
      },
      {
        chapter: '4.2', 
        title: 'References and Borrowing',
        snippet: 'References allow you to use a value without taking ownership...',
        relevance: 0.87
      }
    ].filter(result => 
      result.title.toLowerCase().includes(query.toLowerCase()) ||
      result.snippet.toLowerCase().includes(query.toLowerCase())
    );

    return mockResults;
  }
}