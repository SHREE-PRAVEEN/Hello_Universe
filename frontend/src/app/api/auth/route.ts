import { NextRequest, NextResponse } from 'next/server';
import { z } from 'zod';
import { hashPassword, verifyPassword, generateToken } from '@/lib/crypto';
import { loginSchema, signupSchema } from '@/lib/validations';

// In a real app, these would be in a database
const users = new Map<string, { id: string; email: string; username: string; password: string }>();

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { action } = body;

    if (action === 'login') {
      return handleLogin(body);
    } else if (action === 'signup') {
      return handleSignup(body);
    } else if (action === 'logout') {
      return handleLogout();
    }

    return NextResponse.json(
      { error: 'Invalid action' },
      { status: 400 }
    );
  } catch (error) {
    console.error('Auth error:', error);
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}

async function handleLogin(body: unknown) {
  try {
    const { email, password } = loginSchema.parse(body);

    // Find user (in real app, query database)
    const user = users.get(email);

    if (!user) {
      return NextResponse.json(
        { error: 'Invalid credentials' },
        { status: 401 }
      );
    }

    // Verify password
    const isValid = await verifyPassword(password, user.password);

    if (!isValid) {
      return NextResponse.json(
        { error: 'Invalid credentials' },
        { status: 401 }
      );
    }

    // Generate token
    const token = await generateToken({ userId: user.id, email: user.email });

    // Set cookie
    const response = NextResponse.json({
      success: true,
      user: {
        id: user.id,
        email: user.email,
        username: user.username,
      },
    });

    response.cookies.set('auth-token', token, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'lax',
      maxAge: 60 * 60 * 24 * 7, // 7 days
      path: '/',
    });

    return response;
  } catch (error) {
    if (error instanceof z.ZodError) {
      return NextResponse.json(
        { error: 'Validation failed', details: error.errors },
        { status: 400 }
      );
    }
    throw error;
  }
}

async function handleSignup(body: unknown) {
  try {
    const { email, password, name } = signupSchema.parse(body);

    // Check if user exists
    if (users.has(email)) {
      return NextResponse.json(
        { error: 'User already exists' },
        { status: 409 }
      );
    }

    // Hash password
    const hashedPassword = await hashPassword(password);

    // Create user
    const userId = crypto.randomUUID();
    const user = {
      id: userId,
      email,
      username: name,
      password: hashedPassword,
    };

    users.set(email, user);

    // Generate token
    const token = await generateToken({ userId, email });

    // Set cookie
    const response = NextResponse.json({
      success: true,
      user: {
        id: userId,
        email,
        username: name,
      },
    });

    response.cookies.set('auth-token', token, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'lax',
      maxAge: 60 * 60 * 24 * 7, // 7 days
      path: '/',
    });

    return response;
  } catch (error) {
    if (error instanceof z.ZodError) {
      return NextResponse.json(
        { error: 'Validation failed', details: error.errors },
        { status: 400 }
      );
    }
    throw error;
  }
}

function handleLogout() {
  const response = NextResponse.json({ success: true });

  response.cookies.set('auth-token', '', {
    httpOnly: true,
    secure: process.env.NODE_ENV === 'production',
    sameSite: 'lax',
    maxAge: 0,
    path: '/',
  });

  return response;
}

export async function GET(request: NextRequest) {
  const token = request.cookies.get('auth-token')?.value;

  if (!token) {
    return NextResponse.json(
      { error: 'Not authenticated' },
      { status: 401 }
    );
  }

  // In a real app, verify the token and get user from database
  return NextResponse.json({
    authenticated: true,
    // user data would come from token verification
  });
}
