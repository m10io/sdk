-- Replace Iron bank AUTH0 entities with Omega bank AUTH0
UPDATE public.contacts
SET user_id = 'auth0|628d4d28dc4a31006e430976'
WHERE user_id = 'auth0|6227946b65653d0068e10403';