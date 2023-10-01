using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace Utils
{
    public class CollisionDetection : MonoBehaviour
    {
        [Header("References")]
        public LayerMask WhatIsBlocks;

        [Header("Horizontal Collision")]
        [SerializeField] private Vector2 _rightColOffset = Vector2.zero;
        [SerializeField] private Vector2 _leftColOffset = Vector2.zero;

        [Range(0f, 1f)]
        [SerializeField] private float _horizontalColRadius = .5f;

        [Header("Vertical Collision")]
        [SerializeField] private Vector2 _bottomColSize = Vector2.zero;
        [SerializeField] private Vector2 _bottomColOffset = Vector2.zero;

        [Space]

        public bool IsOnRightWall = false;
        public bool IsOnLeftWall = false;
        public bool IsOnWall = false;
        public bool IsGroundedEarly = false;

        public float BottomColYSize;

        public int WallSide;

        [Space]

        public bool IsGrounded = false;

        private void FixedUpdate()
        {
            IsOnRightWall = Physics2D.OverlapCircle((Vector2) transform.position + _rightColOffset, _horizontalColRadius, WhatIsBlocks);
            IsOnLeftWall = Physics2D.OverlapCircle((Vector2) transform.position + _leftColOffset, _horizontalColRadius, WhatIsBlocks);

            IsOnWall = IsOnLeftWall || IsOnRightWall;
            WallSide = IsOnRightWall ? -1 : 1;

            IsGrounded = Physics2D.OverlapBox((Vector2) transform.position + _bottomColOffset, _bottomColSize, 0f, WhatIsBlocks);
            IsGroundedEarly = Physics2D.OverlapBox((Vector2) transform.position + new Vector2(_bottomColOffset.x, _bottomColOffset.y - .12f), new Vector2(_bottomColSize.x, _bottomColSize.y + .2f), 0f, WhatIsBlocks);
        }

        private void OnDrawGizmosSelected()
        {
            Gizmos.color = Color.red;
            Gizmos.DrawWireSphere((Vector2) transform.position + _rightColOffset, _horizontalColRadius);
            Gizmos.DrawWireSphere((Vector2) transform.position + _leftColOffset, _horizontalColRadius);

            Gizmos.color = Color.blue;
            Gizmos.DrawWireCube((Vector2) transform.position + _bottomColOffset, _bottomColSize);
        }
    }
}