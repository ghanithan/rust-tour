export class ExerciseManager {
  constructor() {
    this.exercises = [];
    this.currentExercise = null;
    this.ui = null; // Will be set by platform
  }
  
  setUI(ui) {
    this.ui = ui;
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

  async saveAllFiles(path, files) {
    try {
      const [chapter, exercise] = path.split('/');
      const response = await fetch(`/api/exercises/${chapter}/${exercise}/files`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ files }),
      });

      if (!response.ok) {
        throw new Error(`Failed to save files: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Error saving files:', error);
      throw error;
    }
  }

  async createFile(path, filePath, content = '') {
    try {
      const [chapter, exercise] = path.split('/');
      const response = await fetch(`/api/exercises/${chapter}/${exercise}/file`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ path: filePath, content }),
      });

      if (!response.ok) {
        throw new Error(`Failed to create file: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Error creating file:', error);
      throw error;
    }
  }

  async deleteFile(path, filePath) {
    try {
      const [chapter, exercise] = path.split('/');
      const response = await fetch(`/api/exercises/${chapter}/${exercise}/file`, {
        method: 'DELETE',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ path: filePath }),
      });

      if (!response.ok) {
        throw new Error(`Failed to delete file: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Error deleting file:', error);
      throw error;
    }
  }

  async autoSaveAllFiles(path) {
    if (!this.ui || !this.ui.getAllModifiedFiles) {
      console.warn('UI not available for auto-save');
      return { success: true, savedCount: 0 };
    }
    
    const modifiedFiles = this.ui.getAllModifiedFiles();
    if (modifiedFiles.length === 0) {
      return { success: true, savedCount: 0 };
    }
    
    try {
      await this.saveAllFiles(path, modifiedFiles);
      
      // Mark all models as saved
      modifiedFiles.forEach(file => {
        const model = this.ui.fileModels.get(file.path);
        if (model) {
          // Reset the version tracking to mark as saved
          model.pushStackElement();
        }
      });
      
      return { success: true, savedCount: modifiedFiles.length };
    } catch (error) {
      console.error('Auto-save failed:', error);
      return { success: false, error: error.message };
    }
  }

  async runExercise(path) {
    try {
      // Auto-save all modified files first
      const saveResult = await this.autoSaveAllFiles(path);
      if (!saveResult.success) {
        throw new Error(`Failed to save files: ${saveResult.error}`);
      }
      
      const [chapter, exercise] = path.split('/');
      const response = await fetch(`/api/exercises/${chapter}/${exercise}/run`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error(`Failed to run exercise: ${response.statusText}`);
      }

      return {
        ...await response.json(),
        savedCount: saveResult.savedCount
      };
    } catch (error) {
      console.error('Error running exercise:', error);
      throw error;
    }
  }

  async testExercise(path) {
    try {
      // Auto-save all modified files first
      const saveResult = await this.autoSaveAllFiles(path);
      if (!saveResult.success) {
        throw new Error(`Failed to save files: ${saveResult.error}`);
      }
      
      const [chapter, exercise] = path.split('/');
      const response = await fetch(`/api/exercises/${chapter}/${exercise}/test`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error(`Failed to test exercise: ${response.statusText}`);
      }

      return {
        ...await response.json(),
        savedCount: saveResult.savedCount
      };
    } catch (error) {
      console.error('Error testing exercise:', error);
      throw error;
    }
  }

  async checkExercise(path) {
    try {
      // Auto-save all modified files first
      const saveResult = await this.autoSaveAllFiles(path);
      if (!saveResult.success) {
        throw new Error(`Failed to save files: ${saveResult.error}`);
      }
      
      const [chapter, exercise] = path.split('/');
      const response = await fetch(`/api/exercises/${chapter}/${exercise}/check`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error(`Failed to check exercise: ${response.statusText}`);
      }

      return {
        ...await response.json(),
        savedCount: saveResult.savedCount
      };
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