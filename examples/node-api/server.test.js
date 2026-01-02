const request = require('supertest');
const app = require('./server');

describe('Codex Node.js API', () => {
  beforeEach(() => {
    // Reset users array before each test
    app.locals = app.locals || {};
    app.locals.users = [
      { id: 1, name: 'Alice', email: 'alice@example.com' },
      { id: 2, name: 'Bob', email: 'bob@example.com' }
    ];
  });

  describe('GET /', () => {
    it('should return welcome message', async () => {
      const response = await request(app).get('/');
      expect(response.status).toBe(200);
      expect(response.body.message).toBe('Codex Node.js API Example');
    });
  });

  describe('GET /users', () => {
    it('should return all users', async () => {
      const response = await request(app).get('/users');
      expect(response.status).toBe(200);
      expect(response.body.success).toBe(true);
      expect(Array.isArray(response.body.data)).toBe(true);
      expect(response.body.count).toBeGreaterThan(0);
    });
  });

  describe('GET /users/:id', () => {
    it('should return user by id', async () => {
      const response = await request(app).get('/users/1');
      expect(response.status).toBe(200);
      expect(response.body.success).toBe(true);
      expect(response.body.data.name).toBe('Alice');
    });

    it('should return 404 for non-existent user', async () => {
      const response = await request(app).get('/users/999');
      expect(response.status).toBe(404);
      expect(response.body.success).toBe(false);
    });
  });

  describe('POST /users', () => {
    it('should create new user', async () => {
      const newUser = { name: 'Charlie', email: 'charlie@example.com' };
      const response = await request(app)
        .post('/users')
        .send(newUser);

      expect(response.status).toBe(201);
      expect(response.body.success).toBe(true);
      expect(response.body.data.name).toBe('Charlie');
      expect(response.body.data.id).toBeDefined();
    });

    it('should return 400 for missing required fields', async () => {
      const response = await request(app)
        .post('/users')
        .send({ name: 'Test' }); // missing email

      expect(response.status).toBe(400);
      expect(response.body.success).toBe(false);
    });
  });

  describe('PUT /users/:id', () => {
    it('should update existing user', async () => {
      const updatedUser = { name: 'Alice Updated', email: 'alice.updated@example.com' };
      const response = await request(app)
        .put('/users/1')
        .send(updatedUser);

      expect(response.status).toBe(200);
      expect(response.body.success).toBe(true);
      expect(response.body.data.name).toBe('Alice Updated');
    });

    it('should return 404 for non-existent user', async () => {
      const response = await request(app)
        .put('/users/999')
        .send({ name: 'Test' });

      expect(response.status).toBe(404);
      expect(response.body.success).toBe(false);
    });
  });

  describe('DELETE /users/:id', () => {
    it('should delete existing user', async () => {
      const response = await request(app).delete('/users/1');
      expect(response.status).toBe(200);
      expect(response.body.success).toBe(true);
      expect(response.body.data.id).toBe(1);
    });

    it('should return 404 for non-existent user', async () => {
      const response = await request(app).delete('/users/999');
      expect(response.status).toBe(404);
      expect(response.body.success).toBe(false);
    });
  });

  describe('Error handling', () => {
    it('should return 404 for unknown routes', async () => {
      const response = await request(app).get('/unknown-route');
      expect(response.status).toBe(404);
      expect(response.body.success).toBe(false);
    });
  });
});