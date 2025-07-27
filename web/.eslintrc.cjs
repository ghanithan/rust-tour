module.exports = {
    env: {
        browser: true,
        es2021: true,
        node: true
    },
    extends: [
        'eslint:recommended'
    ],
    parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module'
    },
    rules: {
        // Disable some common issues in simple projects
        'no-unused-vars': 'warn',
        'no-console': 'off',
        'no-undef': 'warn',
        
        // Relaxed code style for existing codebase
        'indent': 'off', // Too many inconsistencies in existing code
        'quotes': 'off', // Mixed quotes in existing code
        'semi': ['error', 'always'],
        
        // Best practices (relaxed)
        'eqeqeq': 'warn',
        'curly': 'off', // Existing code uses single-line ifs
        'no-var': 'warn',
        'prefer-const': 'warn',
        'no-useless-escape': 'off',
        'no-dupe-class-members': 'warn'
    },
    globals: {
        // Browser globals that might be used
        'monaco': 'readonly',
        'Terminal': 'readonly',
        'FitAddon': 'readonly'
    }
};