export class ExerciseManager {
  constructor() {
    this.exercises = [];
    this.currentExercise = null;
  }

  async init() {
    console.log('Initializing Exercise Manager...');
  }

  async loadExercises() {
    try {
      const response = await fetch('/api/exercises');
      if (!response.ok) {
        throw new Error(`Failed to load exercises: ${response.statusText}`);
      }
      
      this.exercises = await response.json();
      return this.exercises;
    } catch (error) {
      console.error('Error loading exercises:', error);
      throw error;
    }
  }

  async loadExercise(path) {
    try {
      const [chapter, exercise] = path.split('/');
      const response = await fetch(`/api/exercises/${chapter}/${exercise}`);
      
      if (!response.ok) {
        throw new Error(`Failed to load exercise: ${response.statusText}`);
      }
      
      const exerciseData = await response.json();
      this.currentExercise = exerciseData;
      
      return exerciseData;
    } catch (error) {
      console.error('Error loading exercise:', error);
      throw error;
    }
  }

  async saveCode(path, code) {
    try {
      const [chapter, exercise] = path.split('/');
      const response = await fetch(`/api/exercises/${chapter}/${exercise}/code`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ code }),
      });

      if (!response.ok) {
        throw new Error(`Failed to save code: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Error saving code:', error);
      throw error;
    }
  }

  async runExercise(path) {
    try {
      const [chapter, exercise] = path.split('/');
      const response = await fetch(`/api/exercises/${chapter}/${exercise}/run`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error(`Failed to run exercise: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Error running exercise:', error);
      throw error;
    }
  }

  async testExercise(path) {
    try {
      const [chapter, exercise] = path.split('/');
      const response = await fetch(`/api/exercises/${chapter}/${exercise}/test`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error(`Failed to test exercise: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Error testing exercise:', error);
      throw error;
    }
  }

  async checkExercise(path) {
    try {
      const [chapter, exercise] = path.split('/');
      const response = await fetch(`/api/exercises/${chapter}/${exercise}/check`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error(`Failed to check exercise: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Error checking exercise:', error);
      throw error;
    }
  }

  getExerciseById(id) {
    return this.exercises.find(ex => ex.id === id);
  }

  getExercisesByChapter(chapter) {
    return this.exercises.filter(ex => ex.chapter === chapter);
  }

  getNextExercise(currentPath) {
    const currentIndex = this.exercises.findIndex(ex => ex.path === currentPath);
    if (currentIndex !== -1 && currentIndex < this.exercises.length - 1) {
      return this.exercises[currentIndex + 1];
    }
    return null;
  }

  getPreviousExercise(currentPath) {
    const currentIndex = this.exercises.findIndex(ex => ex.path === currentPath);
    if (currentIndex > 0) {
      return this.exercises[currentIndex - 1];
    }
    return null;
  }
}